import scala.io.Source
import scala.annotation.*

object Advent {
  val opMoveBase = 'A'.toInt
  val playerMoveBase = 'X'.toInt
  def parse(file: String): List[(Int, Int)] = {
    val lines: List[String] = Source.fromFile(file).getLines().toList
    lines.map(_.split(" ")
         .toList
         .map(_.head)
         match {
           case List(opMove, playerMove) =>
             (opMove.toInt - opMoveBase,
              playerMove.toInt - playerMoveBase)
           case _ => throw new RuntimeException
         })
  }
  def scoreRound(round: (Int, Int)): Int = {
    round match {
      case (opMove, playerMove) => {
        val baseScore = 1 + playerMove
        val diff = (opMove - playerMove) match {
          case d: Int if d < 0 => d + 3
          case d: Int => d
        }
        val moveScore = diff match {
          case 0 => 3
          case 1 => 0
          case _ => 6
        }
        baseScore + moveScore
      }
    }
  }
  def scoreRounds(rounds: List[(Int, Int)]) =
    rounds.foldLeft(0)((acc, round) =>  acc + scoreRound(round))
}

@main
def main() = {
  val inputFile = "input.txt"
  val rounds = Advent.parse(inputFile)
  // Part 1
  println(Advent.scoreRounds(rounds))
  // Part 2
  val rounds2 = for (round <- rounds) yield {
    round match {
      case (opMove, playerMove) => {
        playerMove match {
          case 0 => (opMove, (opMove + 2) % 3)
          case 1 => (opMove, opMove)
          case _ => (opMove, (opMove + 1) % 3)
        }
      }
    }
  }
  println(Advent.scoreRounds(rounds2))
}
