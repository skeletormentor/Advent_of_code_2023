import Base.isless

struct Card
    value::Int
    label::Char
end

struct Hand
    cards::Vector{Card}
end

isless(c1::Card, c2::Card) = c1.value < c2.value

cardmapping = Dict('A'=> 14, 'K'=> 13, 'Q'=> 12, 'J'=> 11, 'T'=> 10, '9'=> 9,
'8'=> 8, '7'=> 7, '6'=> 6, '5'=> 5, '4'=> 4, '3'=> 3, '2'=> 2)

Card(label::Char) = Card(cardmapping[label], label)