local math = require("math")
local raug = require("raug")

local two_pi = 2.0 * math.pi

local graph = raug.graph_builder()

local out1 = graph:output()
local out2 = graph:output()

local sr = graph:sample_rate()

local pa = graph:phase_accum()
pa:input(0):connect(sr:recip():output(0))
pa = pa % 1.0

local freq1 = raug.param()
local freq2 = raug.param()
local freq3 = raug.param()

freq1:set(440.0)
freq2:set(220.0)
freq3:set(880.0)

local sine1 = (pa * two_pi * freq1):sin()
local sine2 = (pa * two_pi * freq2 + sine1):sin()
local sine3 = (pa * two_pi * freq3 + sine2):sin()

local final = sine3 * 0.2

final:output(0):connect(out1:input(0))
final:output(0):connect(out2:input(0))

local runtime = graph:build_runtime()

local handle = runtime:run()

raug.sleep(1.0)

handle:stop()
