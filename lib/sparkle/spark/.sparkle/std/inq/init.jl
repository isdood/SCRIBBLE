module Inq

export inquire, inspect, describe

function inquire(obj)
    typeof(obj)
end

function inspect(obj)
    fieldnames(typeof(obj))
end

function describe(obj)
    println("Type: ", typeof(obj))
    println("Fields: ", fieldnames(typeof(obj)))
    println("Methods: ", methods(typeof(obj)))
end

end
