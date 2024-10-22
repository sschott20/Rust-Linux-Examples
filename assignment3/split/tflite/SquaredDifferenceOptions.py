# automatically generated by the FlatBuffers compiler, do not modify

# namespace: tflite

import flatbuffers
from flatbuffers.compat import import_numpy
np = import_numpy()

class SquaredDifferenceOptions(object):
    __slots__ = ['_tab']

    @classmethod
    def GetRootAs(cls, buf, offset=0):
        n = flatbuffers.encode.Get(flatbuffers.packer.uoffset, buf, offset)
        x = SquaredDifferenceOptions()
        x.Init(buf, n + offset)
        return x

    @classmethod
    def GetRootAsSquaredDifferenceOptions(cls, buf, offset=0):
        """This method is deprecated. Please switch to GetRootAs."""
        return cls.GetRootAs(buf, offset)
    @classmethod
    def SquaredDifferenceOptionsBufferHasIdentifier(cls, buf, offset, size_prefixed=False):
        return flatbuffers.util.BufferHasIdentifier(buf, offset, b"\x54\x46\x4C\x33", size_prefixed=size_prefixed)

    # SquaredDifferenceOptions
    def Init(self, buf, pos):
        self._tab = flatbuffers.table.Table(buf, pos)

def SquaredDifferenceOptionsStart(builder):
    builder.StartObject(0)

def Start(builder):
    SquaredDifferenceOptionsStart(builder)

def SquaredDifferenceOptionsEnd(builder):
    return builder.EndObject()

def End(builder):
    return SquaredDifferenceOptionsEnd(builder)


class SquaredDifferenceOptionsT(object):

    # SquaredDifferenceOptionsT
    def __init__(self):
        pass

    @classmethod
    def InitFromBuf(cls, buf, pos):
        squaredDifferenceOptions = SquaredDifferenceOptions()
        squaredDifferenceOptions.Init(buf, pos)
        return cls.InitFromObj(squaredDifferenceOptions)

    @classmethod
    def InitFromPackedBuf(cls, buf, pos=0):
        n = flatbuffers.encode.Get(flatbuffers.packer.uoffset, buf, pos)
        return cls.InitFromBuf(buf, pos+n)

    @classmethod
    def InitFromObj(cls, squaredDifferenceOptions):
        x = SquaredDifferenceOptionsT()
        x._UnPack(squaredDifferenceOptions)
        return x

    # SquaredDifferenceOptionsT
    def _UnPack(self, squaredDifferenceOptions):
        if squaredDifferenceOptions is None:
            return

    # SquaredDifferenceOptionsT
    def Pack(self, builder):
        SquaredDifferenceOptionsStart(builder)
        squaredDifferenceOptions = SquaredDifferenceOptionsEnd(builder)
        return squaredDifferenceOptions
