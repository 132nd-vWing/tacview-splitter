#!/usr/bin/env ipython


with open('Tacview-20200419-200244-DCS-OPUF_MSN18.txt.acmi') as fd:
    tacview_raw_lines = fd.readlines()

fb = open('tacview_blue.txt.acmi', 'w')
fr = open('tacview_red.txt.acmi', 'w')
fv = open('tacview_violet.txt.acmi', 'w')
    
for i, line in enumerate(tacview_raw_lines):
    if not line[0] == '#':  # header is everything before the first '#'
        fb.write(line)
        fr.write(line)
        fv.write(line)
    else:
        break

tacview_raw_lines = tacview_raw_lines[i:]  # remove the header, we don't need it anymore

blue_ids = []
red_ids = []
# violet = neutral faction, used for chaffs, flares, decoys and shrapnel
# we can't decide easily which faction they belong to
# we would need to find the blue or red object with the least distance 
# to violet objects around their spawn time
violet_ids = []  
undecided_ids = []

for line in tacview_raw_lines:
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

    if id_ in blue_ids:
        fb.write(line)
    elif id_ in red_ids:
        fr.write(line)
    elif id_ in violet_ids:
        fv.write(line)
    else:  # id_ == 'both', has timestamps
        fb.write(line)
        fr.write(line)
        fv.write(line)

fb.close()
fr.close()
fv.close()

if len(undecided_ids) != 0:
    print('There were units that are neither BLUEFOR, REDFOR nor NEUTRAL. Please investigate.')
    print(undecided_ids)

