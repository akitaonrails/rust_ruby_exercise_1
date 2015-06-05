def find_actors(filename, skip_lines, target_movie)
  actors = []
  actor = nil
  File.foreach(filename).with_index do |line, line_num|
    next if line_num < 239
    line.encode!(line.encoding, 'binary', invalid: :replace, undef: :replace)
    if line.strip == ''
      actor = nil
      next
    end
    slices = line.split(/\t/)
    actor_buffer = slices.first
    movie        = slices.last
    if actor.nil? && !actor_buffer.nil? && actor_buffer != ''
      actor = actor_buffer
    end
    if !movie.nil? && movie.include?(target_movie)
      actors << actor unless actors.include?(actor)
    end
  end
  actors.join("\n")
end

