# Notes

Welcome to my notes! The following is a collection of math notes
and related subjects.
These notes are organized using the [stellar](https://github.com/paolobettelini/stellar)
software.

# Website preview

![universe preview](./media/universe.png)
![course preview](./media/course.png)

# How to run
Install [stellar](https://github.com/paolobettelini/stellar)
and compile all the snippets.
```bash
cd source
./compile
cd ..
```
This script uses `tectonic` to compile all the `tex` file and then runs `stellar-cli`
to generate the snippets and import them in the database.
To compile the `nannou` snippets you also need `wasm-pack` and `npm`.
Make sure to open the script and change your configurations. A mongoDB database is needed.
It is advisable to set the `CARGO_TARGET_DIR` variable so that
the rust projects share the same target folder.

Then, open the web server
```bash
stellar-cli web --data data/ --connection-url "mongodb://localhost"
```
Go to [localhost:8080/universe/math](http://localhost:8080/universe/math).

# Cite me
```bib
@online{paolobettelini,
  author    = {Paolo Bettelini},
  title     = {notes},
  year      = {2024},
  publisher = {GitHub},
  journal   = {GitHub repository}
}
```