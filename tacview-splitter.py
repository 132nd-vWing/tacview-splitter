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
    descriptors = Descriptors(filenames)
    # replicate the header from the input file into the output files
    tacview_lines_no_header = move_header_to_output_files(tacview_lines, descriptors)
    undecided_ids = move_content_to_output_files(tacview_lines_no_header, descriptors)
    descriptors.close()
    # sanity check
    if len(undecided_ids) != 0:
        print('There were units that are neither BLUE, RED nor NEUTRAL. Please investigate.')
        print(undecided_ids)


def move_content_to_output_files(tacview_lines_no_header: list[str], descriptors: Descriptors) -> list[str]:
    """
    core routine: process all telemetry lines, put them in the correct output file
    for lines that are time stamps, we put these in all output files
    :param tacview_lines_no_header: tacview telemetry, header removed
    :param descriptors: holding all the opened descriptors
    :return: list of unit ids that belong to neither blue, red, nor violet
    """
    #
    blue_ids, red_ids, violet_ids, undecided_ids = (list() for _ in range(4))
    # violet = neutral faction, used for chaffs, flares, decoys and shrapnel. we can't decide easily which faction
    # they belong to. we would need to find the blue or red object with the least distance to violet objects
    # around their spawn time
    continued = False
    for line in tacview_lines_no_header:
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

        if descriptors.filenames.input.is_zip:
            line_output = line.encode()
        else:
            line_output = line
        # code checker thinks that id_ can be unbound, which it cannot
        # noinspection PyUnboundLocalVariable
        if id_ in blue_ids or id_ == 'both':
            descriptors.blue_txt.write(line_output)
        if id_ in red_ids or id_ == 'both':
            descriptors.red_txt.write(line_output)
        if id_ in violet_ids or id_ == 'both':
            descriptors.violet_txt.write(line_output)

        if line.endswith('\\\n'):
            continued = True
        else:
            continued = False
    return undecided_ids


def move_header_to_output_files(tacview_lines: list[str], descriptors: Descriptors) -> list[str]:
    """
    finds the tacview header in tacview_lines and writes it to the three output files
    afterwards removes the header from the input data and returns the remaining content (actual telemetry)
    :param tacview_lines: content of tacview file with header
    :param descriptors: object of Descriptor class
    :return: content of tacview file without header
    """
    for i, line in enumerate(tacview_lines):
        if descriptors.filenames.input.is_zip:
            line_header = line.encode()
        else:
            line_header = line
        if not line[0] == '#':  # header is everything before the first '#'
            descriptors.blue_txt.write(line_header)
            descriptors.red_txt.write(line_header)
            descriptors.violet_txt.write(line_header)
        else:
            break
    else:
        raise IOError('Tacview file seems to be empty')
    return tacview_lines[i:]


def read_data(filenames: Filenames) -> list[str]:
    """
    get the tacview data out of the file
    :param filenames: object of class Filenames
    :return: file content as list of strings, one line per item
    """
    if filenames.input.is_zip:
        with ZipFile(filenames.input.zip) as fd_zip:
            with fd_zip.open(fd_zip.filelist[0].filename) as fd_tacview:
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


@dataclass
class Descriptors:
    def __init__(self, filenames: Filenames):
        self.filenames = filenames
        if filenames.input.is_zip:
            self._blue_zip = ZipFile(filenames.output.blue.zip, 'w', ZIP_DEFLATED)
            self._red_zip = ZipFile(filenames.output.red.zip, 'w', ZIP_DEFLATED)
            self._violet_zip = ZipFile(filenames.output.violet.zip, 'w', ZIP_DEFLATED)

            self.blue_txt = self._blue_zip.open(filenames.output.blue.txt, 'w')
            self.red_txt = self._red_zip.open(filenames.output.blue.txt, 'w')
            self.violet_txt = self._violet_zip.open(filenames.output.blue.txt, 'w')
        else:
            self._blue_zip = None
            self._red_zip = None
            self._violet_zip = None

            self.blue_txt = open(filenames.output.blue.txt, 'w')
            self.red_txt = open(filenames.output.red.txt, 'w')
            self.violet_txt = open(filenames.output.violet.txt, 'w')

    def close(self):
        self.blue_txt.close()
        self.red_txt.close()
        self.violet_txt.close()

        if self.filenames.input.is_zip:
            self._blue_zip.close()
            self._red_zip.close()
            self._violet_zip.close()


if __name__ == '__main__':
    main()
