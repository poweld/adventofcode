package object Zipper {
  // dp-> Should use sealed top level classes to make pattern matching not require matching _
  // Maybe this isn't necessary w/ case classes or its subclasses
  case class Tree[T](itemOrTreeList: Either[T, List[Tree[T]]])
  object Item {
    def unapply[T](i: Item[T]) = Some(i.item)
  }
  class Item[T](val item: T) extends Tree(Left(item))

  object Section {
    def unapply[T](s: Section[T]) = Some(s.trees)
  }
  class Section[T](val trees: List[Tree[T]] = List()) extends Tree(Right(trees))

  case class Path[T](pathSandwich: (List[Tree[T]], Option[Path[T]], List[Tree[T]]))

  object Node {
    def unapply[T](n: Node[T]) = Some(n.left, n.up, n.right)
  }
  class Node[T](val left: List[Tree[T]],
                val up: Option[Path[T]],
                val right: List[Tree[T]])
    extends Path((left, up, right)) {
  }

  case class Location[T](val tree: Tree[T], val maybePath: Option[Path[T]])

  def goLeft[T](loc: Location[T]) = loc match
    case Location(tree, maybePath) => maybePath match
      case None => throw new RuntimeException("left of top")
      case Some(Node(l::left, up, right)) => Location(l, Some(Node(left, up, tree::right)))
      case Some(Node(Nil, up, right)) => throw new RuntimeException("left of first")
      case _ => throw new RuntimeException("invalid location type")

  def goRight[T](loc: Location[T]) = loc match
    case Location(tree, maybePath) => maybePath match
      case None => throw new RuntimeException("right of top")
      case Some(Node(left, up, r::right)) => Location(r, Some(Node(tree::left, up, right)))
      case Some(Node(Nil, up, right)) => throw new RuntimeException("right of last")
      case _ => throw new RuntimeException("invalid location type")

  def goUp[T](loc: Location[T]) = loc match
    case Location(tree, maybePath) => maybePath match
      case None => throw new RuntimeException("up of top")
      case Some(Node(left, up, right)) => Location(Tree(Right(left.reverse ::: tree::right)), up)
      case _ => throw new RuntimeException("invalid location type")

  def goDown[T](loc: Location[T]) = loc match
    case Location(tree, maybePath) => tree match
      case Item(_) => throw new RuntimeException("down of item")
      case Section[T](t1::trees) => Location(t1, Some(Node(List(), maybePath, trees)))
      case _ => throw new RuntimeException("down of empty")

  def change[T](loc: Location[T], tree: Tree[T]) = Location(tree, loc.maybePath)
  def insertRight[T](loc: Location[T], r: Tree[T]) = loc match
    case Location(tree, maybePath) => maybePath match
      case None => throw new RuntimeException("insert of top")
      case Some(Node(left, up, right)) => Location(tree, Some(Node(left, up, r::right)))
      case _ => throw new RuntimeException("invalid location type")
  def insertLeft[T](loc: Location[T], l: Tree[T]) = loc match
    case Location(tree, maybePath) => maybePath match
      case None => throw new RuntimeException("insert of top")
      case Some(Node(left, up, right)) => Location(tree, Some(Node(l::left, up, right)))
      case _ => throw new RuntimeException("invalid location type")
  def insertDown[T](loc: Location[T], d: Tree[T]) = loc match
    case Location(tree, maybePath) => tree match
      case Item(_) => throw new RuntimeException("down of item")
      case Section[T](sons) => Location(d, Some(Node(List(), maybePath, sons)))
      case _ => throw new RuntimeException("down of empty")
}

abstract class Test {
  import Zipper._
  val s: Item[Int]
}

@main
def main() = {
  import Zipper._
  // val s = Section(List(Section(List(Item("a"), Item("*"), Item("b"))),
  //                      Item("+"),
  //                      Section(List(Item("c"), Item("*"), Item("d")))))
  // println(s)
  
  val root: Section[String] = Section()
  val fs = insertDown(Location(root, None), Item("test"))
  println(fs)
  //val s: Section[Int]
  //val n: Int
}
