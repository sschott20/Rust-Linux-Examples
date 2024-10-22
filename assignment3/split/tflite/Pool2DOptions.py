# automatically generated by the FlatBuffers compiler, do not modify

# namespace: tflite

import flatbuffers
from flatbuffers.compat import import_numpy
np = import_numpy()

class Pool2DOptions(object):
    __slots__ = ['_tab']

    @classmethod
    def GetRootAs(cls, buf, offset=0):
        n = flatbuffers.encode.Get(flatbuffers.packer.uoffset, buf, offset)
        x = Pool2DOptions()
        x.Init(buf, n + offset)
        return x

    @classmethod
    def GetRootAsPool2DOptions(cls, buf, offset=0):
        """This method is deprecated. Please switch to GetRootAs."""
        return cls.GetRootAs(buf, offset)
    @classmethod
    def Pool2DOptionsBufferHasIdentifier(cls, buf, offset, size_prefixed=False):
        return flatbuffers.util.BufferHasIdentifier(buf, offset, b"\x54\x46\x4C\x33", size_prefixed=size_prefixed)

    # Pool2DOptions
    def Init(self, buf, pos):
        self._tab = flatbuffers.table.Table(buf, pos)

    # Pool2DOptions
    def Padding(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(4))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int8Flags, o + self._tab.Pos)
        return 0

    # Pool2DOptions
    def StrideW(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(6))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

    # Pool2DOptions
    def StrideH(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(8))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

    # Pool2DOptions
    def FilterWidth(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(10))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

    # Pool2DOptions
    def FilterHeight(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(12))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int32Flags, o + self._tab.Pos)
        return 0

    # Pool2DOptions
    def FusedActivationFunction(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(14))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int8Flags, o + self._tab.Pos)
        return 0

def Pool2DOptionsStart(builder):
    builder.StartObject(6)

def Start(builder):
    Pool2DOptionsStart(builder)

def Pool2DOptionsAddPadding(builder, padding):
    builder.PrependInt8Slot(0, padding, 0)

def AddPadding(builder, padding):
    Pool2DOptionsAddPadding(builder, padding)

def Pool2DOptionsAddStrideW(builder, strideW):
    builder.PrependInt32Slot(1, strideW, 0)

def AddStrideW(builder, strideW):
    Pool2DOptionsAddStrideW(builder, strideW)

def Pool2DOptionsAddStrideH(builder, strideH):
    builder.PrependInt32Slot(2, strideH, 0)

def AddStrideH(builder, strideH):
    Pool2DOptionsAddStrideH(builder, strideH)

def Pool2DOptionsAddFilterWidth(builder, filterWidth):
    builder.PrependInt32Slot(3, filterWidth, 0)

def AddFilterWidth(builder, filterWidth):
    Pool2DOptionsAddFilterWidth(builder, filterWidth)

def Pool2DOptionsAddFilterHeight(builder, filterHeight):
    builder.PrependInt32Slot(4, filterHeight, 0)

def AddFilterHeight(builder, filterHeight):
    Pool2DOptionsAddFilterHeight(builder, filterHeight)

def Pool2DOptionsAddFusedActivationFunction(builder, fusedActivationFunction):
    builder.PrependInt8Slot(5, fusedActivationFunction, 0)

def AddFusedActivationFunction(builder, fusedActivationFunction):
    Pool2DOptionsAddFusedActivationFunction(builder, fusedActivationFunction)

def Pool2DOptionsEnd(builder):
    return builder.EndObject()

def End(builder):
    return Pool2DOptionsEnd(builder)


class Pool2DOptionsT(object):

    # Pool2DOptionsT
    def __init__(self):
        self.padding = 0  # type: int
        self.strideW = 0  # type: int
        self.strideH = 0  # type: int
        self.filterWidth = 0  # type: int
        self.filterHeight = 0  # type: int
        self.fusedActivationFunction = 0  # type: int

    @classmethod
    def InitFromBuf(cls, buf, pos):
        pool2Doptions = Pool2DOptions()
        pool2Doptions.Init(buf, pos)
        return cls.InitFromObj(pool2Doptions)

    @classmethod
    def InitFromPackedBuf(cls, buf, pos=0):
        n = flatbuffers.encode.Get(flatbuffers.packer.uoffset, buf, pos)
        return cls.InitFromBuf(buf, pos+n)

    @classmethod
    def InitFromObj(cls, pool2Doptions):
        x = Pool2DOptionsT()
        x._UnPack(pool2Doptions)
        return x

    # Pool2DOptionsT
    def _UnPack(self, pool2Doptions):
        if pool2Doptions is None:
            return
        self.padding = pool2Doptions.Padding()
        self.strideW = pool2Doptions.StrideW()
        self.strideH = pool2Doptions.StrideH()
        self.filterWidth = pool2Doptions.FilterWidth()
        self.filterHeight = pool2Doptions.FilterHeight()
        self.fusedActivationFunction = pool2Doptions.FusedActivationFunction()

    # Pool2DOptionsT
    def Pack(self, builder):
        Pool2DOptionsStart(builder)
        Pool2DOptionsAddPadding(builder, self.padding)
        Pool2DOptionsAddStrideW(builder, self.strideW)
        Pool2DOptionsAddStrideH(builder, self.strideH)
        Pool2DOptionsAddFilterWidth(builder, self.filterWidth)
        Pool2DOptionsAddFilterHeight(builder, self.filterHeight)
        Pool2DOptionsAddFusedActivationFunction(builder, self.fusedActivationFunction)
        pool2Doptions = Pool2DOptionsEnd(builder)
        return pool2Doptions
