# serbitoros
A simple static fileserver built in Rust with a rest API using the Rocket framework,

Before compiling with rustc, be sure to run the generate_indexes.py script to generate the index.html files for each directory/subdirectory of the fileserver. This isn't necessary for the functioning of the Rest API, but when users navigate to a folder, the index.html will display the subfolders and files present at that level, like a traditional file system.
