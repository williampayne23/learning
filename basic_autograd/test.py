import unittest
from Tensor import Tensor

class TestTensorGrad(unittest.TestCase):
  def test_sum(self):
    x = Tensor([[1],[2],[3],[4]], requires_grad=True)
    y = x.sum() 
    y.backward()
    self.assertEqual(x.grad.data.tolist(), [[1],[1],[1],[1]])

  def test_add(self):
    x = Tensor([[1],[2],[3],[4]], requires_grad=True)
    y = (x + x).sum()
    y.backward()
    self.assertEqual(x.grad.data.tolist(), [[2],[2],[2],[2]])
  
  def test_sub(self):
    x = Tensor([[1],[2],[3],[4]], requires_grad=True)
    y = (x - x - x).sum()
    y.backward()
    self.assertEqual(x.grad.data.tolist(), [[-1],[-1],[-1],[-1]])

  def test_mult(self):
    a = Tensor([[1,2,3,4]], requires_grad=True)
    b = Tensor([[1],[2],[5],[4]], requires_grad=True)
    y = (a * b)
    z = y.sum()
    z.backward()
    self.assertEqual(a.grad.data.tolist(), [[1,2,5,4]])
  
  def test_pow(self):
    a = Tensor([[1,2,3,4]], requires_grad=True)
    y = (a ** 3)
    z = y.sum()
    z.backward()
    self.assertEqual(a.grad.data.tolist(), [[3,12,27,48]])

class TestTensorArith(unittest.TestCase):
  def test_sum(self):
    x = Tensor([[1],[2],[3],[4]], requires_grad=True)
    y = x.sum() 
    self.assertEqual(y.data.tolist(), 10)

  def test_add(self):
    x = Tensor([[1],[2],[3],[4]], requires_grad=True)
    y = (x + x)
    self.assertEqual(y.data.tolist(), [[2],[4],[6],[8]])
  
  def test_sub(self):
    a = Tensor([[2],[3],[4],[5]], requires_grad=True)
    b = Tensor([[1],[2],[3],[4]], requires_grad=True)
    y = (a - b)
    self.assertEqual(y.data.tolist(), [[1],[1],[1],[1]])

  def test_mult(self):
    a = Tensor([[1,2,3,4]], requires_grad=True)
    b = Tensor([[1],[2],[3],[4]], requires_grad=True)
    y = (a * b)
    self.assertEqual(y.data.tolist(), [[30]])
  
  def test_pow(self):
    a = Tensor([[1,2,3,4]], requires_grad=True)
    y = (a ** 2)
    self.assertEqual(y.data.tolist(), [[1,4,9,16]])

if __name__ == '__main__':
    unittest.main()