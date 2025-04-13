local raug = require("raug")

local sine = raug.sine_oscillator(440.0)

local mix = sine * 0.2

raug.audio_output(mix)
raug.audio_output(mix)

raug.run_for(5.0)