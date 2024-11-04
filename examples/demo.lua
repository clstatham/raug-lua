local raug = require("raug")

local graph = raug.graph_builder()

local out1 = graph:output()
local out2 = graph:output()

local sine = graph:sine_osc()

local freq = sine:input("frequency"):param()
freq:set(440.0)

sine = sine * 0.2

sine:output(0):connect(out1:input(0))
sine:output(0):connect(out2:input(0))

local runtime = graph:build_runtime()

local handle = runtime:run()

raug.sleep(1.0)

freq:set(880.0)

raug.sleep(1.0)

freq:set(220.0)

raug.sleep(1.0)

handle:stop()
