# automatically generated by the FlatBuffers compiler, do not modify

# namespace: tflite

import flatbuffers
from flatbuffers.compat import import_numpy
np = import_numpy()

class SoftmaxOptions(object):
    __slots__ = ['_tab']

    @classmethod
    def GetRootAs(cls, buf, offset=0):
        n = flatbuffers.encode.Get(flatbuffers.packer.uoffset, buf, offset)
        x = SoftmaxOptions()
        x.Init(buf, n + offset)
        return x

    @classmethod
    def GetRootAsSoftmaxOptions(cls, buf, offset=0):
        """This method is deprecated. Please switch to GetRootAs."""
        return cls.GetRootAs(buf, offset)
    @classmethod
    def SoftmaxOptionsBufferHasIdentifier(cls, buf, offset, size_prefixed=False):
        return flatbuffers.util.BufferHasIdentifier(buf, offset, b"\x54\x46\x4C\x33", size_prefixed=size_prefixed)

    # SoftmaxOptions
    def Init(self, buf, pos):
        self._tab = flatbuffers.table.Table(buf, pos)

    # SoftmaxOptions
    def Beta(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(4))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Float32Flags, o + self._tab.Pos)
        return 0.0

def SoftmaxOptionsStart(builder):
    builder.StartObject(1)

def Start(builder):
    SoftmaxOptionsStart(builder)

def SoftmaxOptionsAddBeta(builder, beta):
    builder.PrependFloat32Slot(0, beta, 0.0)

def AddBeta(builder, beta):
    SoftmaxOptionsAddBeta(builder, beta)

def SoftmaxOptionsEnd(builder):
    return builder.EndObject()

def End(builder):
    return SoftmaxOptionsEnd(builder)


class SoftmaxOptionsT(object):

    # SoftmaxOptionsT
    def __init__(self):
        self.beta = 0.0  # type: float

    @classmethod
    def InitFromBuf(cls, buf, pos):
        softmaxOptions = SoftmaxOptions()
        softmaxOptions.Init(buf, pos)
        return cls.InitFromObj(softmaxOptions)

    @classmethod
    def InitFromPackedBuf(cls, buf, pos=0):
        n = flatbuffers.encode.Get(flatbuffers.packer.uoffset, buf, pos)
        return cls.InitFromBuf(buf, pos+n)

    @classmethod
    def InitFromObj(cls, softmaxOptions):
        x = SoftmaxOptionsT()
        x._UnPack(softmaxOptions)
        return x

    # SoftmaxOptionsT
    def _UnPack(self, softmaxOptions):
        if softmaxOptions is None:
            return
        self.beta = softmaxOptions.Beta()

    # SoftmaxOptionsT
    def Pack(self, builder):
        SoftmaxOptionsStart(builder)
        SoftmaxOptionsAddBeta(builder, self.beta)
        softmaxOptions = SoftmaxOptionsEnd(builder)
        return softmaxOptions
