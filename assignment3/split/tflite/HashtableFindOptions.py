# automatically generated by the FlatBuffers compiler, do not modify

# namespace: tflite

import flatbuffers
from flatbuffers.compat import import_numpy
np = import_numpy()

class HashtableFindOptions(object):
    __slots__ = ['_tab']

    @classmethod
    def GetRootAs(cls, buf, offset=0):
        n = flatbuffers.encode.Get(flatbuffers.packer.uoffset, buf, offset)
        x = HashtableFindOptions()
        x.Init(buf, n + offset)
        return x

    @classmethod
    def GetRootAsHashtableFindOptions(cls, buf, offset=0):
        """This method is deprecated. Please switch to GetRootAs."""
        return cls.GetRootAs(buf, offset)
    @classmethod
    def HashtableFindOptionsBufferHasIdentifier(cls, buf, offset, size_prefixed=False):
        return flatbuffers.util.BufferHasIdentifier(buf, offset, b"\x54\x46\x4C\x33", size_prefixed=size_prefixed)

    # HashtableFindOptions
    def Init(self, buf, pos):
        self._tab = flatbuffers.table.Table(buf, pos)

def HashtableFindOptionsStart(builder):
    builder.StartObject(0)

def Start(builder):
    HashtableFindOptionsStart(builder)

def HashtableFindOptionsEnd(builder):
    return builder.EndObject()

def End(builder):
    return HashtableFindOptionsEnd(builder)


class HashtableFindOptionsT(object):

    # HashtableFindOptionsT
    def __init__(self):
        pass

    @classmethod
    def InitFromBuf(cls, buf, pos):
        hashtableFindOptions = HashtableFindOptions()
        hashtableFindOptions.Init(buf, pos)
        return cls.InitFromObj(hashtableFindOptions)

    @classmethod
    def InitFromPackedBuf(cls, buf, pos=0):
        n = flatbuffers.encode.Get(flatbuffers.packer.uoffset, buf, pos)
        return cls.InitFromBuf(buf, pos+n)

    @classmethod
    def InitFromObj(cls, hashtableFindOptions):
        x = HashtableFindOptionsT()
        x._UnPack(hashtableFindOptions)
        return x

    # HashtableFindOptionsT
    def _UnPack(self, hashtableFindOptions):
        if hashtableFindOptions is None:
            return

    # HashtableFindOptionsT
    def Pack(self, builder):
        HashtableFindOptionsStart(builder)
        hashtableFindOptions = HashtableFindOptionsEnd(builder)
        return hashtableFindOptions
