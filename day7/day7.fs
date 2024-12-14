open System
open System.IO

let parseItem(item: string) =
    let parts = item.Split(":")
    let value = parts.[0] |> int64
    let numbers = parts.[1].Trim().Split(" ") |> Array.map(fun item -> int64 item) |> Seq.toList
    (value, numbers)

let rec results operators (acc: int64) (rem: int64 list) =
    match rem with
    | [] -> [ acc ]
    | head :: tail -> operators |> List.map(fun op -> results operators (op acc head) tail) |> List.concat

let isEquationTrue operators (item: int64 * int64 list) =
    let (value, numbers) = item

    results operators numbers.[0] numbers.[1..] |> Seq.contains value

let concat (lhs: int64) (rhs: int64) =
    int64 (string lhs + string rhs)

[<EntryPoint>]
let main args =
    let path = args |> Array.head

    let input =
        File.ReadAllText(path).Split("\n")
        |> Array.map(fun item -> parseItem(item))

    let part1 =
        input
        |> Seq.filter(fun item -> isEquationTrue [ (+); (*) ] (item))
        |> Seq.map(fun item -> item |> fst)
        |> Seq.sum
    printfn "Part 1 solution is %d" part1

    let part2 =
        input
        |> Seq.filter(fun item -> isEquationTrue [ (+); (*); concat ] (item))
        |> Seq.map(fun item -> item |> fst)
        |> Seq.sum
    printfn "Part 2 solution is %d" part2

    0
