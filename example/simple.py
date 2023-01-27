from wot_datfile_parser_py import DatfileParser

# You can use this same parser for any number of datfiles
parser = DatfileParser()

with open("19011713064132879.dat", "rb") as file:
    battle = parser.parse(file)