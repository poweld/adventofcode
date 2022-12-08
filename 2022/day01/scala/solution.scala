import scala.io.Source
import scala.annotation.*

object Advent {
  def parse(file: String) = {
    val lines: List[String] = Source.fromFile(file).getLines().toList

    @tailrec
    def parseRec(lines: List[String],
                 groups: List[List[Int]] = List(),
                 group: List[Int] = List()): List[List[Int]] = {
      lines match {
        case head :: tail if head.isEmpty =>
          parseRec(tail, group :: groups, List())
        case head :: tail =>
          parseRec(tail, groups, head.toInt :: group)
        case _ => group :: groups
      }
    }

    parseRec(lines)
  }
}

@main
def solution() = {
  val inputFile = "input.txt"
  val elves = Advent.parse(inputFile)
  val elvesCaloriesDesc = elves.map(_.sum).sorted.reverse
  println(elvesCaloriesDesc.head)
  println(elvesCaloriesDesc.slice(0, 3).sum)
}
