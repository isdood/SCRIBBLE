module Clone

export clone, deep_clone

function clone(obj)
    copy(obj)
end

function deep_clone(obj)
    deepcopy(obj)
end

end
