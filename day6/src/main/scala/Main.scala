import scala.io.Source
import scala.annotation.tailrec
import scala.collection.parallel.CollectionConverters._

type Position = (Int, Int)
type Direction = (Int, Int)

def loadFile(path: String): List[String] =
  val file = Source.fromFile(path)
  val output = file.getLines().toList

  file.close

  output

def findStartPoint(input: List[String]): Position =
  input.zipWithIndex.flatMap { case (row, y) =>
    row.zipWithIndex.collect { case ('^', x) => (y, x) }
  }.headOption.getOrElse(throw new NoSuchElementException("No start point found"))

def applyDirection(current: Position, direction: Direction): Position =
  (current._1 + direction._1, current._2 + direction._2)

def canContinue(position: Position, bounds: Position): Boolean =
  position._1 >= 0 && position._1 < bounds._1 && position._2 >= 0 && position._2 < bounds._2

def switchDirection(direction: Direction): Direction =
  direction match
    case (-1,  0) => ( 0,  1)
    case ( 1,  0) => ( 0, -1)
    case ( 0,  1) => ( 1,  0)
    case ( 0, -1) => (-1,  0)

def part1(input: List[String]) =
  val startPoint = findStartPoint(input)
  val bounds = (input.length, input.head.length)

  def move(current: Position, direction: Direction): Int =
    def move(visited: Set[(Position)], current: Position, direction: Direction): Int =
      val position = applyDirection(current, direction)

      if !canContinue(position, bounds) then visited.size + 1
      else input(position._1)(position._2) match
        case '#' => move(visited + current, applyDirection(current, switchDirection(direction)), switchDirection(direction))
        case _   => move(visited + current, position, direction)
      
    move(Set.empty, current, direction)
    
  val count = move(startPoint, (-1, 0))
  println("Part 1 solution is " + count)

def generateObstacles(input: List[String], guard: Position): List[Position] =
  input.zipWithIndex.flatMap { case (row, y) =>
    row.zipWithIndex.collect { case (_, x) => (y, x) }
  }.filterNot(_ == guard).filterNot { case (y, x) => input(y)(x) == '#' }

// this is not working correctly and needs to be fixed
def part2(input: List[String]) =
  val startPoint = findStartPoint(input)
  val bounds = (input.length, input.head.length)

  val obstacles = generateObstacles(input, startPoint)

  def move(current: Position, direction: Direction): Int =
    @tailrec
    def move(turns: Set[(Position, Position)], current: Position, direction: Direction, obstacle: Position): Int =
      val position = applyDirection(current, direction)

      if !canContinue(position, bounds) then return 0

      val switchedDirection = switchDirection(direction)
      val turn = applyDirection(current, switchedDirection)

      if (position == obstacle || input(position._1)(position._2) == '#') && turns.contains((current, turn)) then return 1

      if position == obstacle
        then move(turns + ((current, turn)), turn, switchedDirection, obstacle)
      else input(position._1)(position._2) match
        case '#' => move(turns + ((current, turn)), turn, switchedDirection, obstacle)
        case _   => move(turns, position, direction, obstacle)

    obstacles.toList.par.map(move(Set.empty, startPoint, (-1, 0), _)).sum
    
  val count = move(startPoint, (-1, 0))

  println("Part 2 solution is " + count)

@main def main(path: String, others: String*): Unit =
  val input = loadFile(path)

  part1(input)
  part2(input)
