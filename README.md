This is a re-implementation of the original [sink example](https://babeltrace.org/docs/v2.0/libbabeltrace2/example-simple-sink-cmp-cls.html).

# Building it
Before building make sure you have Babeltrace2 and its dev library installed:

```bash
sudo apt install Babeltrace2 libbabeltrace2-dev
```

The final building step is currently done from CMake. So to build it:
```bash
mkdir build && cd build
cmake -DCMAKE_BUILD_TYPE=Debug .. && cmake --build .
```

You can then use it with the commands:
```bash
babeltrace2 --plugin-path=build/ /path/to/ctf/trace --component=sink.epitome.output
```