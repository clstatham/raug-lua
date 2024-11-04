local math = require("math")
local raug = require("raug")

local graph = raug.graph_builder()

local out1 = graph:output()
local out2 = graph:output()

local sr = graph:sample_rate()

local pa = graph:phase_accum()

pa:input(0):connect(sr:recip():output(0))

local freq = raug.param()
freq:set(440.0)

local sine = (pa * 2.0 * math.pi * freq):sin()

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
