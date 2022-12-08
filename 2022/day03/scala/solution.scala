import scala.io.Source
import scala.annotation.*

object Advent {
  def priority(item: Char): Int = item match
    case i if 'a' to 'z' contains i => i - 'a' + 1
    case i => i - 'A' + 27
  def toCompartments(rucksack: String): (String, String) =
    rucksack.splitAt(rucksack.length / 2)
  def parse(file: String) = {
    val lines: List[String] = Source.fromFile(file).getLines().toList
    lines
  }
  def chunk[T](lst: List[T], size: Int): List[List[T]] = {
    @tailrec
    def chunkRec[T](lst: List[T],
                    chunks: List[List[T]] = List(),
                    chunk: List[T] = List()): List[List[T]] = lst match
      case head :: tail if chunk.length == size =>
        chunkRec(tail, chunk.reverse :: chunks, List(head))
      case head :: tail =>
        chunkRec(tail, chunks, head :: chunk)
      case Nil => (chunk :: chunks).reverse
    chunkRec(lst)
  }
}

@main
def main() = {
  import Advent._
  val inputFile = "input.txt"
  val parsed = parse(inputFile)
  val part1 = parsed.map(toCompartments)
                    .map((c1, c2) => (c1.toSet, c2.toSet))
                    .map((s1, s2) => s1.intersect(s2))
                    .map(_.toList)
                    .map(_.head)
                    .map(priority)
                    .sum
  println(part1)
  val part2 = chunk(parsed, 3)
                .map(chunk => {
                  val sets = chunk.map(_.toSet)
                  sets.tail.foldLeft(sets.head)(_.intersect(_))
                })
                .map(_.toList)
                .map(_.head)
                .map(priority)
                .sum
  println(part2)
}
