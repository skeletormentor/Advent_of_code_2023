### A Pluto.jl notebook ###
# v0.19.36

using Markdown
using InteractiveUtils

# ╔═╡ 6812b28d-3fce-4dfd-a7ce-0dae037c1c6d
struct Item
	value::Int
	range::UnitRange
	y::Int
end

# ╔═╡ 5deef660-a99f-11ee-04ff-879927efe650
example = split(raw"""467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..""")

# ╔═╡ dd63b7b4-6efd-45c0-8ddb-399d5afffb89
function get_surrounding_coords(item::Item, sizeoftable)
	max_x, max_y = sizeoftable
	border = Tuple{Int, Int}[]
	for j in [item.y - 1, item.y + 1]
		for i in first(item.range)-1:last(item.range)+1
			if (1 <= i <= max_x) && (1 <= j <= max_y)
				push!(border, (i, j))
			end
		end
	end
	if 1 < first(item.range)
		push!(border, (first(item.range) - 1, item.y))
	end
	if first(item.range) <= max_x
		push!(border, (last(item.range) + 1, item.y))
	end
	return border
end

# ╔═╡ 62863571-4b70-400d-ad83-539d1d156100
function parsed(lines)
	rc = length(example)
	cc = length(example[1])
	symbols = Tuple{Int, Int}[]
	gears = Item[]
	items = Item[]
	for row in 1:rc
		num_matches = findall(r"(\d+)", lines[row])
		for m in num_matches
			push!(items, Item(parse(Int, lines[row][m]), m, row))
		end
		for column in 1:cc
			char = lines[row][column]
			if char != '.' && !isdigit(char)
				if char == '*'
					push!(gears, Item(0, column:column+1, row))
				end
				push!(symbols, (column, row))
			end
		end
	end
	return items, gears, symbols, (cc, rc)
end

# ╔═╡ 01fc9b01-b2ef-4ff6-a296-4ac73de9b68d
items, gears, symbols, sizeoftable = parsed(example)

# ╔═╡ 216569ca-1573-435c-b758-d554a0cae40c
function get_sum_of_items(items, symbols, sizeoftable)
	sum_of_items = 0
	for item in items
		surr = get_surrounding_coords(item, sizeoftable)
		for sym in symbols
			if sym in surr
				sum_of_items += item.value
			end
		end
	end
	return sum_of_items
end

# ╔═╡ d6cbbb24-40f9-4c74-b6ef-7d09c5517ba9
get_sum_of_items(items, symbols, sizeoftable)

# ╔═╡ Cell order:
# ╠═6812b28d-3fce-4dfd-a7ce-0dae037c1c6d
# ╠═5deef660-a99f-11ee-04ff-879927efe650
# ╠═dd63b7b4-6efd-45c0-8ddb-399d5afffb89
# ╠═62863571-4b70-400d-ad83-539d1d156100
# ╠═01fc9b01-b2ef-4ff6-a296-4ac73de9b68d
# ╠═216569ca-1573-435c-b758-d554a0cae40c
# ╠═d6cbbb24-40f9-4c74-b6ef-7d09c5517ba9
