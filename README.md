rust-mysql
==========

mysql quick and dirty binding for rust language.

* **Build instruction**

To build rust-mysql you need rust version >= 0.8, you can get it here (for the latest version): <https://github.com/mozilla/rust> .

Now just run `make`, `libmysql-[,,,].so` will build into build/lib directories.
For building examples, you need to add the location of `libmysql-[,,,].so` to your `LIBRARY_PATH` in your env. And run `make examples`. Examples are build into `./build/examples`.




