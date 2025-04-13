local math = require("math")
local raug = require("raug")

local two_pi = 2.0 * math.pi

local sr = raug.sample_rate()

local pa = raug.phase_accumulator(sr:recip())
pa = pa % 1.0

local freq1 = 440.0
local freq2 = 220.0
local freq3 = 880.0

local sine1 = raug.sin(pa * two_pi * freq1)
local sine2 = raug.sin(pa * two_pi * freq2 + sine1)
local sine3 = raug.sin(pa * two_pi * freq3 + sine2)

local final = sine3 * 0.2

raug.audio_output(final)
raug.audio_output(final)

raug.run_for(5.0)