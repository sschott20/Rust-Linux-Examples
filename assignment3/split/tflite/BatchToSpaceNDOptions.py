# automatically generated by the FlatBuffers compiler, do not modify

# namespace: tflite

import flatbuffers
from flatbuffers.compat import import_numpy
np = import_numpy()

class BatchToSpaceNDOptions(object):
    __slots__ = ['_tab']

    @classmethod
    def GetRootAs(cls, buf, offset=0):
        n = flatbuffers.encode.Get(flatbuffers.packer.uoffset, buf, offset)
        x = BatchToSpaceNDOptions()
        x.Init(buf, n + offset)
        return x

    @classmethod
    def GetRootAsBatchToSpaceNDOptions(cls, buf, offset=0):
        """This method is deprecated. Please switch to GetRootAs."""
        return cls.GetRootAs(buf, offset)
    @classmethod
    def BatchToSpaceNDOptionsBufferHasIdentifier(cls, buf, offset, size_prefixed=False):
        return flatbuffers.util.BufferHasIdentifier(buf, offset, b"\x54\x46\x4C\x33", size_prefixed=size_prefixed)

    # BatchToSpaceNDOptions
    def Init(self, buf, pos):
        self._tab = flatbuffers.table.Table(buf, pos)

def BatchToSpaceNDOptionsStart(builder):
    builder.StartObject(0)

def Start(builder):
    BatchToSpaceNDOptionsStart(builder)

def BatchToSpaceNDOptionsEnd(builder):
    return builder.EndObject()

def End(builder):
    return BatchToSpaceNDOptionsEnd(builder)


class BatchToSpaceNDOptionsT(object):

    # BatchToSpaceNDOptionsT
    def __init__(self):
        pass

    @classmethod
    def InitFromBuf(cls, buf, pos):
        batchToSpaceNdoptions = BatchToSpaceNDOptions()
        batchToSpaceNdoptions.Init(buf, pos)
        return cls.InitFromObj(batchToSpaceNdoptions)

    @classmethod
    def InitFromPackedBuf(cls, buf, pos=0):
        n = flatbuffers.encode.Get(flatbuffers.packer.uoffset, buf, pos)
        return cls.InitFromBuf(buf, pos+n)

    @classmethod
    def InitFromObj(cls, batchToSpaceNdoptions):
        x = BatchToSpaceNDOptionsT()
        x._UnPack(batchToSpaceNdoptions)
        return x

    # BatchToSpaceNDOptionsT
    def _UnPack(self, batchToSpaceNdoptions):
        if batchToSpaceNdoptions is None:
            return

    # BatchToSpaceNDOptionsT
    def Pack(self, builder):
        BatchToSpaceNDOptionsStart(builder)
        batchToSpaceNdoptions = BatchToSpaceNDOptionsEnd(builder)
        return batchToSpaceNdoptions
