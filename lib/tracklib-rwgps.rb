require "tracklib-rwgps/version"
require "rutie"

module Tracklib
  Rutie.new(:tracklib_rwgps).init 'Init_Tracklib', __dir__
end

module TracklibRWGPS
  Rutie.new(:tracklib_rwgps).init 'Init_Tracklib_RWGPS', __dir__
end
