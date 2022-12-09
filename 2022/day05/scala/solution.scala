import scala.io.Source
import scala.annotation.*

import scala.collection.mutable.Stack

object Advent {
  // class Stack[T](val lst: List[T] = List()) {
  //   def push(item: T) = Stack(lst :+ item)
  //   def pop() = Stack(lst.dropRight(1))
  //   override def toString() = lst.toString()
  // }
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
  def toStacks(lines: List[String]) = {
    val (stackLines, countLine) = (lines.dropRight(1), lines.last)
    val stackCount = countLine.filter(_ != ' ').length
    val stacks = 0 until stackCount map { _ => Stack[Char]() }
    chunk(stackLines.reverse(0).toList, 4)
    for {
      stackLine <- stackLines.reverse
      (chnk, i) <- chunk(stackLine.toList, 4).zipWithIndex
    } chnk(1) match {
      case ' ' =>
      case c => stacks(i).push(c)
    }
    stacks
  }
  def toDirectives(lines: List[String]): List[List[Int]] = {
    for (line <- lines)
      yield for {
        (word, index) <- line.split(" ").toList.zipWithIndex
          if index % 2 != 0
      } yield word.toInt
  }
  def parse(file: String) = {
    val lines: List[String] = Source.fromFile(file).getLines().toList
    val (stackLines, directiveLines) = lines.splitAt(lines.indexWhere(_.isEmpty)) match
      case (s, d) => (s, d.drop(1)) // Get rid of the empty line
    (toStacks(stackLines), toDirectives(directiveLines))
  }
}

@main
def main() = {
  import Advent._
  val inputFile = "input.txt"
  {
    val parsed = parse(inputFile)
    val (stacks, directives) = parsed
    for (directive <- directives) directive match
      case List(count, from, to) => {
        for (_ <- 0 until count)
          stacks(to - 1).push(stacks(from - 1).pop())
      }
      case _ => throw new RuntimeException
    val part1 = String(stacks.map(_.pop).toArray)
    println(part1)
  }
  {
    val parsed = parse(inputFile)
    val (stacks, directives) = parsed
    for (directive <- directives) directive match
      case List(count, from, to) => {
        //println(s"$stacks")
        //println(s"from: $from, to: $to, count: $count")
        (0 until count).map(_ => stacks(from - 1).pop())
                       .reverse
                       .foreach(stacks(to - 1).push(_))
      }
      case _ => throw new RuntimeException
    val part2 = String(stacks.map(_.pop).toArray)
    println(part2)
  }
}
