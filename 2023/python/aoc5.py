import fileinput
import bisect
import re
import typing
import os

def debug(value: typing.Any):
    if os.environ.get('DEBUG', None):
        print(value)

class Mapping(typing.NamedTuple):
    source: int
    dest: int
    length: int

class Range(typing.NamedTuple):
    start: int
    length: int

class SparseMap:
    source: str
    destination: str
    mappings: list[Mapping]

    def __init__(self, source: str, destination: str):
        self.source = source
        self.destination = destination
        self.mappings = []
    
    def add_mapping(self, source_start: int, dest_start: int, length: int) -> None:
        self.mappings.append(Mapping(source_start, dest_start, length))

    def finalize(self) -> None:
        self.mappings.sort(key=lambda m: m.source)
    
    def convert(self, source_id: int) -> int:
        mapping_pos = bisect.bisect_left(self.mappings, source_id, key=lambda m: m.source)
        
        if mapping_pos < len(self.mappings) and source_id == self.mappings[mapping_pos].source:
            # id is first value of a mapping
            return self.mappings[mapping_pos].dest
        if mapping_pos > 0:
            #between mappings or part of mapping before position
            mapping = self.mappings[mapping_pos-1]
            if source_id >= mapping.source and source_id < mapping.source + mapping.length:
                # debug(f"{source_id} in {mapping}")
                return mapping.dest + (source_id - mapping.source)
        
        # No mapping found
        return source_id
    
    def convert_range(self, source_range: Range) -> list[Range]:
        remaining_source: Range = source_range
        mapped_ranges: list[Range] = []

        while remaining_source.length > 0:
            source_id = remaining_source.start
            mapping_pos = bisect.bisect_left(self.mappings, source_id, key=lambda m: m.source)

            if mapping_pos == 0:
                mapping = self.mappings[mapping_pos]
            else:
                mapping = self.mappings[mapping_pos]
            
            if source_id < mapping.source:
                # Before first mapping
                available_range = mapping.source - source_id
                if available_range < remaining_source.length:
                    mapped_ranges.append(Range(source_id, available_range))
                    remaining_source = Range(source_id + available_range, remaining_source.length - available_range)
                else:
                    mapped_ranges.append(remaining_source)
                    remaining_source = Range(source_id + remaining_source.length, 0)
            elif source_id < mapping.source + mapping.length:


        raise Exception("not implemented")

seeds: list[str] = []
seed_ranges: list[Range] = []
current_map: typing.Optional[SparseMap] = None
maps: dict[str, SparseMap] = {}
for line in fileinput.input():
    if not seeds and line.startswith("seeds: "):
        #part 1
        seeds = line[len("seeds: "):].split()
        #part 2
        for i in range(len(seeds)//2):
            seed_ranges.append(Range(start=int(seeds[i*2]), length=int(seeds[i*2+1])))
    elif current_map:
        if len(line) > 1:
            dest_start, source_start, length = [int(p) for p in line.split()]
            current_map.add_mapping(source_start, dest_start, length)
        else:
            current_map.finalize()
            maps[current_map.source] = current_map
            current_map = None
    else:
        m = re.match(r"(\w+)-to-(\w+) map:", line)
        if m:
            current_map = SparseMap(m.group(1), m.group(2))

if current_map:
    current_map.finalize()
    maps[current_map.source] = current_map
    current_map = None


def seed_location(seed: int) -> int:
    type = "seed"
    id = seed

    debug(f"seed:{seed}")
    while type != "location":
        m = maps[type]
        type = m.destination
        id = m.convert(id)
        debug(f"| {type}:{id}")
    
    return id

def min_seed_range_location(seed_range: Range) -> int:
    
    raise Exception

# part 1
minimum_location = min([seed_location(int(s)) for s in seeds])
# part 2
for range in seed_ranges:
    print(range)

print(f"Part 1: {minimum_location}")