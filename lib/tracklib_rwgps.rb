require "tracklib_rwgps/version"
require "rutie"

module Tracklib
  Rutie.new(:tracklib_rwgps).init 'Init_Tracklib', __dir__
end

module TracklibRwgps
  Rutie.new(:tracklib_rwgps).init 'Init_Tracklib_Rwgps', __dir__
end
