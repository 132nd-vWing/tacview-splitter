#!/usr/bin/env ipython

from __future__ import annotations
from dataclasses import dataclass
from os import listdir as os_listdir
from typing import Tuple
from zipfile import ZipFile, ZIP_DEFLATED


EXTENSION_TXT = '.txt.acmi'
EXTENSION_ZIP = '.zip.acmi'


def main():
    filename_input, is_zip = find_input_file()
    print('Processing ' + str(filename_input))
    filenames = Filenames(filename_input, is_zip)
    tacview_lines = read_data(filenames)

    # set up all the file descriptors we will need
    if is_zip:
        fd_blue_zip = ZipFile(filenames.output.blue.zip, 'w', ZIP_DEFLATED)
        fd_red_zip = ZipFile(filenames.output.red.zip, 'w', ZIP_DEFLATED)
        fd_violet_zip = ZipFile(filenames.output.violet.zip, 'w', ZIP_DEFLATED)

        fd_blue_txt = fd_blue_zip.open(filenames.output.blue.txt, 'w')
        fd_red_txt = fd_red_zip.open(filenames.output.blue.txt, 'w')
        fd_violet_txt = fd_violet_zip.open(filenames.output.blue.txt, 'w')
    else:
        fd_blue_txt = open(filenames.output.blue.txt, 'w')
        fd_red_txt = open(filenames.output.red.txt, 'w')
        fd_violet_txt = open(filenames.output.violet.txt, 'w')

    # replicate the header from the input file into the output files
    for i, line in enumerate(tacview_lines):
        if is_zip:
            line_header = line.encode()
        else:
            line_header = line
        if not line[0] == '#':  # header is everything before the first '#'
            fd_blue_txt.write(line_header)
            fd_red_txt.write(line_header)
            fd_violet_txt.write(line_header)
        else:
            break
    else:
        raise IOError('Tacview file seems to be empty')

    tacview_lines = tacview_lines[i:]  # remove the header, we don't need it anymore
    blue_ids = []
    red_ids = []
    # violet = neutral faction, used for chaffs, flares, decoys and shrapnel
    # we can't decide easily which faction they belong to
    # we would need to find the blue or red object with the least distance
    # to violet objects around their spawn time
    violet_ids = []
    undecided_ids = []

    # core routine: process all the lines, put them in the correct output file
    continued = False
    for line in tacview_lines:
        # tacview introduced continued lines, signified by a single backslash at EOL
        # example: DCS briefing is copied into tacview file (begins with `0,Briefing=`)
        # if the line was not continued, we need to extract the unit ID from the line
        # otherwise we reuse the ID from the previous loop
        if not continued:
            # the first time a unit appears it has the Color in its line.
            # The first part of the line (before the first comma) is the unique ID of the unit.
            if line[0] == '#':  # check if line is a time stamp
                id_ = 'both'
            elif line[0] == '-':  # negative id is used to indicate a destroyed unit
                id_ = line[1:].strip()
            else:  # otherwise it is a standard unit line
                id_, rest = line.split(',', 1)
            if 'Color=Red' in line:
                red_ids.append(id_)
            elif 'Color=Blue' in line:
                blue_ids.append(id_)
            elif 'Color=Violet' in line:
                violet_ids.append(id_)
            elif 'Color=' in line:
                undecided_ids.append(id_)

        if is_zip:
            line_output = line.encode()
        else:
            line_output = line
        # code checker thinks that id_ can be unbound because, which it cannot
        # noinspection PyUnboundLocalVariable
        if id_ in blue_ids:
            fd_blue_txt.write(line_output)
        elif id_ in red_ids:
            fd_red_txt.write(line_output)
        elif id_ in violet_ids:
            fd_violet_txt.write(line_output)
        else:  # id_ == 'both', has timestamps
            fd_blue_txt.write(line_output)
            fd_red_txt.write(line_output)
            fd_violet_txt.write(line_output)

        if line.endswith('\\\n'):
            continued = True
        else:
            continued = False
    # main work completed, close all descriptors
    fd_blue_txt.close()
    fd_red_txt.close()
    fd_violet_txt.close()
    if is_zip:
        # there is no way the variable is unbound when we are in this branch
        # noinspection PyUnboundLocalVariable
        fd_blue_zip.close()
        # noinspection PyUnboundLocalVariable
        fd_red_zip.close()
        # noinspection PyUnboundLocalVariable
        fd_violet_zip.close()
    # sanity check
    if len(undecided_ids) != 0:
        print('There were units that are neither BLUE, RED nor NEUTRAL. Please investigate.')
        print(undecided_ids)


def read_data(filenames: Filenames) -> list[str]:
    if filenames.input.is_zip:
        with ZipFile(filenames.input.zip) as fd_zip:
            with fd_zip.open(filenames.input.txt) as fd_tacview:
                tacview_lines_binary = fd_tacview.readlines()
        tacview_lines_ascii = []
        for line in tacview_lines_binary:
            tacview_lines_ascii.append(line.decode())
    else:
        with open(filenames.input.txt) as fd_tacview:
            tacview_lines_ascii = fd_tacview.readlines()
    return tacview_lines_ascii


def find_input_file() -> Tuple[str, bool]:
    all_files = os_listdir('.')
    for filename in all_files:
        filename_lower = filename.lower()
        if filename_lower.endswith(EXTENSION_ZIP):
            is_zip = True
            break
        elif filename_lower.endswith(EXTENSION_TXT):
            is_zip = False
            break
    else:
        raise FileNotFoundError('Could not find a tacview file in this directory.')
    return filename, is_zip


@dataclass
class Filenames:
    @dataclass
    class _Input:
        def __init__(self, filename_input: str, is_zip: bool):
            self.zip: str = ''
            self.txt: str = ''
            self.no_extension: str = ''
            self.is_zip: bool = is_zip
            if is_zip:
                self.zip = filename_input
                self.txt = filename_input.replace(EXTENSION_ZIP, EXTENSION_TXT)
                self.no_extension = filename_input.replace(EXTENSION_ZIP, '')
            else:
                self.zip = ''
                self.txt = filename_input
                self.no_extension = filename_input.replace(EXTENSION_TXT, '')

    @dataclass
    class _Output:
        @dataclass
        class _Coalition:
            def __init__(self, filename_input_no_extension: str, color: str):
                self.no_extension: str = f'{filename_input_no_extension}_{color}'
                self.zip: str = f'{self.no_extension}{EXTENSION_ZIP}'
                self.txt: str = f'{self.no_extension}{EXTENSION_TXT}'

        def __init__(self, filename_input_no_extension: str):
            self.blue = self._Coalition(filename_input_no_extension, 'blue')
            self.red = self._Coalition(filename_input_no_extension, 'red')
            self.violet = self._Coalition(filename_input_no_extension, 'violet')

    def __init__(self, filename_input: str, is_zip: bool):
        self.input = self._Input(filename_input, is_zip)
        self.output = self._Output(self.input.no_extension)


if __name__ == '__main__':
    main()
