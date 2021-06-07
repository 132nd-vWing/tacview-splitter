#!/usr/bin/env ipython

from os import listdir as os_listdir
import zipfile

EXTENSION_TXT = '.txt.acmi'
EXTENSION_ZIP = '.zip.acmi'

is_zip = None

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

filename_input = filename
print('Processing ' + str(filename_input))

if is_zip:
    filename_no_extension = filename_input.replace(EXTENSION_ZIP, '')
else:
    filename_no_extension = filename_input.replace(EXTENSION_TXT, '')

filename_blue_no_extension, filename_red_no_extension, filename_violet_no_extension = \
        (f'{filename_no_extension}_{color}' for color in ('blue', 'red', 'violet'))

if is_zip:
    filename_blue_zip, filename_red_zip, filename_violet_zip = \
        (f'{arg}{EXTENSION_ZIP}' for arg in
         (filename_blue_no_extension, filename_red_no_extension, filename_violet_no_extension)
         )

filename_blue_txt, filename_red_txt, filename_violet_txt = \
    (f'{arg}{EXTENSION_TXT}' for arg in
     (filename_blue_no_extension, filename_red_no_extension, filename_violet_no_extension)
     )

if is_zip:
    with zipfile.ZipFile(filename_input) as fd_zip:
        with fd_zip.open(filename_no_extension + EXTENSION_TXT) as fd_tacview:
            tacview_binary_lines = fd_tacview.readlines()
    tacview_raw_lines = []
    for line in tacview_binary_lines:
        tacview_raw_lines.append(line.decode())
else:
    with open(filename_input) as fd_tacview:
        tacview_raw_lines = fd_tacview.readlines()

if is_zip:
    # noinspection PyUnboundLocalVariable
    fd_blue_zip = zipfile.ZipFile(filename_blue_zip, 'w', zipfile.ZIP_DEFLATED)
    # noinspection PyUnboundLocalVariable
    fd_red_zip = zipfile.ZipFile(filename_red_zip, 'w', zipfile.ZIP_DEFLATED)
    # noinspection PyUnboundLocalVariable
    fd_violet_zip = zipfile.ZipFile(filename_violet_zip, 'w', zipfile.ZIP_DEFLATED)

    fd_blue_txt = fd_blue_zip.open(filename_blue_txt, 'w')
    fd_red_txt = fd_red_zip.open(filename_red_txt, 'w')
    fd_violet_txt = fd_violet_zip.open(filename_violet_txt, 'w')
else:
    fd_blue_txt = open(filename_blue_txt, 'w')
    fd_red_txt = open(filename_red_txt, 'w')
    fd_violet_txt = open(filename_violet_txt, 'w')

for i, line in enumerate(tacview_raw_lines):
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

tacview_raw_lines = tacview_raw_lines[i:]  # remove the header, we don't need it anymore

blue_ids = []
red_ids = []
# violet = neutral faction, used for chaffs, flares, decoys and shrapnel
# we can't decide easily which faction they belong to
# we would need to find the blue or red object with the least distance
# to violet objects around their spawn time
violet_ids = []
undecided_ids = []

continued = False
for line in tacview_raw_lines:
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

fd_blue_txt.close()
fd_red_txt.close()
fd_violet_txt.close()

if is_zip:
    # noinspection PyUnboundLocalVariable
    fd_blue_zip.close()
    # noinspection PyUnboundLocalVariable
    fd_red_zip.close()
    # noinspection PyUnboundLocalVariable
    fd_violet_zip.close()

if len(undecided_ids) != 0:
    print('There were units that are neither BLUE, RED nor NEUTRAL. Please investigate.')
    print(undecided_ids)
