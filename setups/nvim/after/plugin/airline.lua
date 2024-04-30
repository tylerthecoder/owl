vim.g.airline_mode_map = {
    ["__"] = '-',
    ["c"] = 'C',
    ["i"] = 'I',
    ["ic"] = 'I',
    ["ix"] = 'I',
    ["n"] = 'N',
    ["multi"] = 'M',
    ["ni"] = 'N',
    ["no"] = 'N',
    ["R"] = 'R',
    ["Rv"] = 'R',
    ["s"] = 'S',
    ["S"] = 'S',
    ["␓"] = 'S',
    ["t"] = 'T',
    ["v"] = 'V',
    ["V"] = 'V',
    ["␖"] = 'V',
}

vim.g.airline_section_z = '%3l/%L:%3v'

vim.g.airline_statusline_ontop = 0


-- Hide encoding unless it's not utf-8
vim.g['airline#parts#ffenc#skip_expected_string'] = 'utf-8[unix]'
