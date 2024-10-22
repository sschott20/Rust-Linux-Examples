# automatically generated by the FlatBuffers compiler, do not modify

# namespace: tflite

import flatbuffers
from flatbuffers.compat import import_numpy
np = import_numpy()

class DequantizeOptions(object):
    __slots__ = ['_tab']

    @classmethod
    def GetRootAs(cls, buf, offset=0):
        n = flatbuffers.encode.Get(flatbuffers.packer.uoffset, buf, offset)
        x = DequantizeOptions()
        x.Init(buf, n + offset)
        return x

    @classmethod
    def GetRootAsDequantizeOptions(cls, buf, offset=0):
        """This method is deprecated. Please switch to GetRootAs."""
        return cls.GetRootAs(buf, offset)
    @classmethod
    def DequantizeOptionsBufferHasIdentifier(cls, buf, offset, size_prefixed=False):
        return flatbuffers.util.BufferHasIdentifier(buf, offset, b"\x54\x46\x4C\x33", size_prefixed=size_prefixed)

    # DequantizeOptions
    def Init(self, buf, pos):
        self._tab = flatbuffers.table.Table(buf, pos)

def DequantizeOptionsStart(builder):
    builder.StartObject(0)

def Start(builder):
    DequantizeOptionsStart(builder)

def DequantizeOptionsEnd(builder):
    return builder.EndObject()

def End(builder):
    return DequantizeOptionsEnd(builder)


class DequantizeOptionsT(object):

    # DequantizeOptionsT
    def __init__(self):
        pass

    @classmethod
    def InitFromBuf(cls, buf, pos):
        dequantizeOptions = DequantizeOptions()
        dequantizeOptions.Init(buf, pos)
        return cls.InitFromObj(dequantizeOptions)

    @classmethod
    def InitFromPackedBuf(cls, buf, pos=0):
        n = flatbuffers.encode.Get(flatbuffers.packer.uoffset, buf, pos)
        return cls.InitFromBuf(buf, pos+n)

    @classmethod
    def InitFromObj(cls, dequantizeOptions):
        x = DequantizeOptionsT()
        x._UnPack(dequantizeOptions)
        return x

    # DequantizeOptionsT
    def _UnPack(self, dequantizeOptions):
        if dequantizeOptions is None:
            return

    # DequantizeOptionsT
    def Pack(self, builder):
        DequantizeOptionsStart(builder)
        dequantizeOptions = DequantizeOptionsEnd(builder)
        return dequantizeOptions
