public class DynamicArray {
  let defaultCapacity = 10
  var size = 0
  var elements: [Int] = []

  /// 数组大小
  func sizeof() -> Int {
    return size
  }
  /// 是否空数组
  func isEmpty() -> Bool {
    return size == 0
  }
  /// 获取 index 位置元素
  /// - Parameter index: 元素位置
  func get(index: Int) -> Int {
    if index < 0 || index > size { return 0 }
    return elements[index]
  }
}
