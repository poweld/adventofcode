import scala.io.Source
import scala.annotation.*

object Advent {
  implicit def splitToList(s: String, sep: String): List[String] = s.split(sep).toList
  def parse(file: String) = {
    val lines: List[String] = Source.fromFile(file).getLines().toList
    lines.map(_.split(",").toList
               .map(_.split("-").toList
                     .map(_.toInt)).toList
               .map(_ match {
                 case List(start: Int, end: Int) => start to end
                 case _ => throw new RuntimeException
               }))}
  def contains(r1: Range, r2: Range) = r1.contains(r2.start)
                                       && r1.contains(r2.end)
  def overlaps(r1: Range, r2: Range) = r1.contains(r2.start)
                                       || r1.contains(r2.end)
                                       || r2.contains(r1.start)
                                       || r2.contains(r1.end)
}

@main
def main() = {
  import Advent._
  val inputFile = "input.txt"
  val parsed = parse(inputFile).to(LazyList)
  val part1 = parsed
                .map(rs => contains(rs(0), rs(1)) || contains(rs(1), rs(0)))
                .map(if (_) 1 else 0)
                .sum
  println(part1)
  val part2 = parsed
                .map(rs => overlaps(rs(0), rs(1)))
                .map(if (_) 1 else 0)
                .sum
  println(part2)
}
