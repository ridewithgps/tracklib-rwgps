require "tracklib_rwgps/version"
require "rutie"

module TracklibNext
  Rutie.new(:tracklib_rwgps).init 'Init_Tracklib_Next', __dir__
end

module TracklibRwgps
  Rutie.new(:tracklib_rwgps).init 'Init_Tracklib_Rwgps', __dir__
end
