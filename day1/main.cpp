#include <algorithm>
#include <charconv>
#include <cmath>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <ranges>
#include <stdexcept>
#include <string>
#include <string_view>
#include <vector>

using Lists = std::pair<std::vector<std::int64_t>, std::vector<std::int64_t>>;

std::vector<std::string> Split(std::string input)
{
    using std::operator""sv;

    return input
        | std::views::split("   "sv)
        | std::views::take(2)
        | std::ranges::to<std::vector<std::string>>();
}

std::pair<std::int64_t, std::int64_t> ParseNumbers(std::string input)
{
    auto first = std::int64_t{};
    auto second = std::int64_t{};

    const auto strings = Split(std::move(input));

    std::ignore = std::from_chars(strings.front().data(), strings.front().data() + strings.front().size(), first);
    std::ignore = std::from_chars(strings.back().data(), strings.back().data() + strings.back().size(), second);

    return std::make_pair(first, second);
}

Lists LoadFile(const std::filesystem::path& filepath)
{
    auto firstList = std::vector<std::int64_t>{};
    auto secondList = std::vector<std::int64_t>{};

    auto file = std::ifstream{filepath};
    auto input = std::string{};
    while (std::getline(file, input))
    {
        const auto [first, second] = ParseNumbers(std::move(input));
        firstList.push_back(first);
        secondList.push_back(second);
    }

    return std::make_pair(std::move(firstList), std::move(secondList));
}

void Part1(const Lists& input)
{
    auto [firstList, secondList] = input;
    std::ranges::sort(firstList);
    std::ranges::sort(secondList);

    const auto result =
        std::ranges::fold_left(
            std::views::zip_transform(
                [](const auto first, const auto second) -> std::int64_t { return std::abs(first - second); },
                firstList,
                secondList
            ),
            std::int64_t{},
            std::plus{}
    );

    std::cout << "Part 1 solution is " << result << '\n';
}

void Part2(const Lists& input)
{
    const auto& [firstList, secondList] = input;

    const auto result =
        std::ranges::fold_left(
            firstList,
            std::int64_t{},
            [&secondList](const auto acc, const auto id) -> std::int64_t
            {
                return acc + (id * std::ranges::count(secondList, id));
            }
        );
    
    std::cout << "Part 2 solution is " << result << '\n';
}

int main(int argc, char** argv)
{
    if (argc < 2)
    {
        throw std::runtime_error("Provide filepath");
    }

    const auto input = LoadFile(argv[1]);

    Part1(input);
    Part2(input);
}
