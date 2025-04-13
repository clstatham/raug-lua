local math = require("math")
local raug = require("raug")

local sine = raug.sine_oscillator(440.0)

raug.audio_output(sine.out)
raug.audio_output(sine.out)

raug.run_for(1.0)