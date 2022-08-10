local pickers = require('telescope.pickers')
local make_entry = require('telescope.make_entry')
local finders = require('telescope.finders')
local conf = require('telescope.config').values
local dendron = require('namjul.dendron')
-- local actions = require('telescope.actions')
-- local utils = require('dendron/utils')
-- local cmd = require('dendron/cmd')

local M = {}

function M.lookup(opts)
  opts = opts or {}

  opts.cwd = dendron.config.dendron_dir

  local find_command = { 'fd', '--type', 'f' }

  opts.entry_maker = opts.entry_maker or make_entry.gen_from_file(opts)

  pickers.new(opts, {
    prompt_title = 'Lookup',
    finder = finders.new_oneshot_job(find_command, opts),
    previewer = conf.file_previewer(opts),
    sorter = conf.generic_sorter(opts),
  }):find()
end

-- M.lookup({ cwd = '~/Dropbox/dendron' })

return M
