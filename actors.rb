require 'ffi'
require 'benchmark'
require './imdb.rb'

module RustWorld
  extend FFI::Library
  ffi_lib 'target/release/libimdb.so'
  attach_function :ffi_find_actors, [:string, :int, :string], :string
end

filename = 'actors_slice.list'
target_movie = 'The X Factor'

if ENV['RUST'] == '1'
  puts "running Rust/FFI version"
  puts Benchmark.measure {
    puts RustWorld.ffi_find_actors(filename, 239, target_movie)
  }
else
  puts "running pure Ruby version"
  puts Benchmark.measure {
    puts find_actors(filename, 239, target_movie)
  }
end
