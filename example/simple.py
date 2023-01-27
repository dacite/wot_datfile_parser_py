from wot_datfile_parser_py import DatfileParser

parser = DatfileParser()

with open("19011713064132879.dat", "rb") as file:
    battle = parser.parse(file)