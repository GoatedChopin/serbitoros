import os

folder_uni = str('\U0001F4C1')
file_uni = str('\U0001F5CE')


def generate(folder, write_file=True, folder_icon=folder_uni, file_icon=file_uni, sep=" "):
    dirs = []
    files = []
    for path in os.listdir(folder):
        if os.path.isdir(folder + os.sep + path):
            generate(folder + os.sep + path,
                     write_file=write_file,
                     folder_icon=folder_icon,
                     file_icon=file_icon,
                     sep=" ")
            dirs.append(path)
        elif not path.endswith("index.html"):
            files.append(path)
    if write_file:
        with open(folder + "/index.html", "w") as file:
            file.write("<!DOCTYPE html>\n")
            for dir in dirs:
                file.write("<p>{}{}{}</p>\n".format(folder_icon, sep, dir))
            for f in files:
                file.write("<p>{}{}{}</p>\n".format(file_icon, sep, f))
            file.write("</html>")
    else:
        print(dirs)
        print(files)


if __name__ == "__main__":
    generate("src/static", write_file=True, file_icon="|_", sep=" ")
