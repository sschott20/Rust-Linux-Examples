# automatically generated by the FlatBuffers compiler, do not modify

# namespace: tflite

import flatbuffers
from flatbuffers.compat import import_numpy
np = import_numpy()

class MaximumMinimumOptions(object):
    __slots__ = ['_tab']

    @classmethod
    def GetRootAs(cls, buf, offset=0):
        n = flatbuffers.encode.Get(flatbuffers.packer.uoffset, buf, offset)
        x = MaximumMinimumOptions()
        x.Init(buf, n + offset)
        return x

    @classmethod
    def GetRootAsMaximumMinimumOptions(cls, buf, offset=0):
        """This method is deprecated. Please switch to GetRootAs."""
        return cls.GetRootAs(buf, offset)
    @classmethod
    def MaximumMinimumOptionsBufferHasIdentifier(cls, buf, offset, size_prefixed=False):
        return flatbuffers.util.BufferHasIdentifier(buf, offset, b"\x54\x46\x4C\x33", size_prefixed=size_prefixed)

    # MaximumMinimumOptions
    def Init(self, buf, pos):
        self._tab = flatbuffers.table.Table(buf, pos)

def MaximumMinimumOptionsStart(builder):
    builder.StartObject(0)

def Start(builder):
    MaximumMinimumOptionsStart(builder)

def MaximumMinimumOptionsEnd(builder):
    return builder.EndObject()

def End(builder):
    return MaximumMinimumOptionsEnd(builder)


class MaximumMinimumOptionsT(object):

    # MaximumMinimumOptionsT
    def __init__(self):
        pass

    @classmethod
    def InitFromBuf(cls, buf, pos):
        maximumMinimumOptions = MaximumMinimumOptions()
        maximumMinimumOptions.Init(buf, pos)
        return cls.InitFromObj(maximumMinimumOptions)

    @classmethod
    def InitFromPackedBuf(cls, buf, pos=0):
        n = flatbuffers.encode.Get(flatbuffers.packer.uoffset, buf, pos)
        return cls.InitFromBuf(buf, pos+n)

    @classmethod
    def InitFromObj(cls, maximumMinimumOptions):
        x = MaximumMinimumOptionsT()
        x._UnPack(maximumMinimumOptions)
        return x

    # MaximumMinimumOptionsT
    def _UnPack(self, maximumMinimumOptions):
        if maximumMinimumOptions is None:
            return

    # MaximumMinimumOptionsT
    def Pack(self, builder):
        MaximumMinimumOptionsStart(builder)
        maximumMinimumOptions = MaximumMinimumOptionsEnd(builder)
        return maximumMinimumOptions
