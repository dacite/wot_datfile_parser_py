from wot_datfile_parser_py import DatfileParser

# You can use this same parser for any number of datfiles
parser = DatfileParser()

with open("9252658313363554.dat", "rb") as file:
    battle = parser.parse(file)
    print(battle)