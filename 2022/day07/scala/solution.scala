import scala.io.Source
import scala.annotation.*

object Advent {
  def parse(file: String) = {
    val lines: List[String] = Source.fromFile(file).getLines().toList
    lines.map(_.split(" ").toList)
  }
  trait INode (val name: String) {
    def size: Int
  }
  class Dir(
    name: String,
    val children: Map[String, INode] = Map(),
  ) extends INode(name) {
    def size = children.map((name, child) => child match
      case f: File => f.size
      case d: Dir if name == ".." => 0
      case d: Dir => d.size
      case _ => 0
    ).sum
    override def toString: String = s"$name (dir)"
    def root: Dir = {
      @tailrec
      def loop(n: Dir): Dir = n.children.get("..") match
        case None => n
        case Some(n: Dir) => loop(n)
        case _ => throw new RuntimeException(".. is not a Dir")
      loop(this)
    }
    def withParent(pd: Dir) = Dir(name, children.updated("..", pd))
    def add(n: INode) = n match {
      // pretty sure the issue is that when we add a new file or dir
      // the child dirs need to be updated with the new parent object
      case d: Dir => {
        Dir(name, children.updated(d.name, d.withParent(this)))
      }
      case f: File => Dir(name, children.updated(f.name, f))
    }
    def findDirs: List[Dir] = {
      @tailrec
      def loop(acc: List[Dir], toEval: List[Dir]): List[Dir] = {
        // println(s"loop - acc: $acc, toEval: $toEval")
        toEval match {
          case Nil => acc
          case head :: tail =>  {
            val childDirs = head.children.flatMap {
              case ("..", d: Dir) => None
              case (_, d: Dir) => Some(d)
              case _ => None
            }.toList
            loop(head :: acc, childDirs ::: toEval.tail)
          }
        }
      }
      loop(Nil, List(this))
    }
  }
  class File(
    name: String,
    val size: Int,
  ) extends INode(name) {
    override def toString: String = s"$name (file, size=$size)"
  }
}

@main
def main() = {
  import Advent._
  val inputFile = "test_input.txt"
  {
    // Part 1
    val parsed = parse(inputFile)
    //val root = Dir("/")
    //println(root.findDirs.map(_.size))
    //println(parsed)
    val root = parsed.foldLeft(Dir("/"))((cwd, lineParts) => {
      val line = lineParts.mkString(" ")
      println(line)
      val res = lineParts.match {
        case List("$", "cd", d) => d match {
          case "/" => cwd.root
          case d: String => cwd.children(d) match
            case d: Dir => d
            case _ => throw new RuntimeException("cd to a non-directory")
        }
        case List("$", "ls") => cwd
        case List("dir", d: String) => cwd.add(Dir(d))
        case List(size, name) => cwd.add(File(name, size.toInt))
        case _ => cwd
      }
      println(s"cwd: $res, size: ${res.size}")
      res
    }).root
    val dirs = root.findDirs
    println(dirs.zip(dirs.map(_.size)))
  }
  {
    // Part 2
    // val parsed = parse(inputFile)
    // val result = parsed
    // println(result)
  }
}
