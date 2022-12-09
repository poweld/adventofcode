import scala.io.Source
import scala.annotation.*

//import scala.collection.mutable.HashMap
//import scala.collection.immutable.HashMap
import scala.collection.immutable.HashMap

object Advent {
  def parse(file: String) = {
    val lines: List[String] = Source.fromFile(file).getLines().toList
    lines(0).toList
  }
  class Counter[K](val m: HashMap[K, Int]) {
    def add(v: K): Counter[K] = Counter(m.updated(v, get(v) + 1))
    def add(vs: Iterable[K]): Counter[K] = vs.foldLeft(this)(_.add(_))
    def sub(v: K): Counter[K] = Counter(m.updated(v, get(v) - 1))
    def get(v: K): Int = m.getOrElse(v, 0)
    def keys = m.keys
    def values = m.values
    def this() = this(HashMap())
    override def toString = m.toString
  }
  def findUniqueSequenceIndex(chars: List[Char], sequenceLen: Int): Int = {
    @tailrec
    def loop(rest: List[(Char, Int)],
             window: List[(Char, Int)],
             counter: Counter[Char]): Int = {
      // println(s"Window: $window")
      // println(s"Counter: $counter")
      if (counter.values.forall(_ <= 1))
        window.last._2 + 1
      else
        loop(rest.tail,
             window.drop(1) :+ rest.head,
             counter.sub(window.head._1)
                    .add(rest.head._1))
    }
    val charsZippedWithIndex = chars.zipWithIndex
    loop(charsZippedWithIndex.drop(sequenceLen),
         charsZippedWithIndex.take(sequenceLen),
         Counter[Char]().add(chars.take(sequenceLen)))
  }
}

@main
def main() = {
  import Advent._
  val inputFile = "input.txt"
  val windowSize = 4
  {
    val parsed = parse(inputFile)
    val part1 = (findUniqueSequenceIndex(parsed, 4))
    println(part1)
  }
  {
    val parsed = parse(inputFile)
    val part1 = (findUniqueSequenceIndex(parsed, 14))
    println(part1)
  }
}
