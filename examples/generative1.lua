local raug = require('raug')
local math = require('math')

local function midi_to_freq(midi_note)
    local A4 = 440.0
    local A4_midi = 69
    local semitone_ratio = 2 ^ (1 / 12)
    return A4 * (semitone_ratio ^ (midi_note - A4_midi))
end

local function scale_freqs()
    local freqs = {}
    local scale = { 0, 2, 3, 5, 7, 8, 10 }
    local base = 36
    local scale_length = #scale
    local num_octaves = 3 -- Number of octaves to generate

    for octave = 0, num_octaves - 1 do
        for i = 1, scale_length do
            local note = scale[i] + octave * 12 + base
            local freq = midi_to_freq(note)
            table.insert(freqs, freq)
        end
    end

    return freqs
end

local function fm_sine_osc(freq, mod_freq)
    local sr = raug.sample_rate()
    local phase = raug.phase_accumulator(freq / sr)
    return raug.sin(phase * 2.0 * math.pi + mod_freq * 2.0 * math.pi)
end



local function random_tones(rates, ratios, freqs, decays, amps)
    local mast = raug.metro(rates[1])
    local rate = raug.random_choice(mast, rates):unwrap_or(0.0)
    local trig = raug.metro(rate)

    local freq = raug.random_choice(trig, freqs):unwrap_or(440.0)
    local amp_decay = raug.random_choice(trig, decays):unwrap_or(0.0)
    local ratio = raug.random_choice(trig, ratios):unwrap_or(0.0)
    local amp = raug.random_choice(trig, amps):unwrap_or(0.0)

    local amp_env = raug.decay_env(trig, amp_decay)

    local modulator = raug.bl_saw_oscillator(freq * ratio)

    local carrier = fm_sine_osc(freq, modulator * 0.1)

    return carrier * amp_env * amp
end

local function generative1()
    local rates = { 1. / 8., 1. / 4., 1. / 2., 1., 2. }
    local ratios = { 0.25, 0.5, 1.0, 2.0 }
    local freqs = scale_freqs()
    local decays = { 0.02, 0.1, 0.2, 0.5 }
    local amps = { 0.125, 0.25, 0.5, 0.8 }

    return random_tones(rates, ratios, freqs, decays, amps)
end

local function main()
    local mix = generative1()
    for _ = 0, 19 do
        mix = mix + generative1()
    end
    mix = mix * 0.2

    mix = raug.peak_limiter(mix)

    raug.audio_output(mix)
    raug.audio_output(mix)

    raug.run_for(10.0)
end

main()
