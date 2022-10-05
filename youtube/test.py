import csv

with open('songs.csv', 'r', encoding='utf8') as file:
    reader = csv.reader(file)
    for row in reader:
        print(row)
