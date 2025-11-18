(raworder, rawmanuals) = split(read("input/5ex", String), "\n\n")

order = [parse.(Int, split(line, "|")) for line in eachsplit(raworder)]
manuals = [parse.(Int, split(line, ",")) for line in eachsplit(rawmanuals)]

function pagesort(manual)

end