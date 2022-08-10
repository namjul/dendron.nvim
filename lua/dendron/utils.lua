local M = {}

function M.on_stderr_factory(name)
  return vim.schedule_wrap(function(error, data)
    assert(not error, error)
    vim.cmd(string.format("echoerr 'An error occured from running %s: %s'", name, data))
  end)
end

function M.file_exists(path)
  local f = io.open(path, 'r')
  if f ~= nil then
    io.close(f)
    return true
  else
    return false
  end
end

function M.lines_from(path)
  if not M.file_exists(path) then
    return {}
  end
  local lines = {}
  for line in io.lines(path) do
    lines[#lines + 1] = line
  end
  return lines
end

function M.trim(s)
  return (string.gsub(s, '^%s*(.-)%s*$', '%1'))
end

return M
