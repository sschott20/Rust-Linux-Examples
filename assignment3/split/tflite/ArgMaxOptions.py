# automatically generated by the FlatBuffers compiler, do not modify

# namespace: tflite

import flatbuffers
from flatbuffers.compat import import_numpy
np = import_numpy()

class ArgMaxOptions(object):
    __slots__ = ['_tab']

    @classmethod
    def GetRootAs(cls, buf, offset=0):
        n = flatbuffers.encode.Get(flatbuffers.packer.uoffset, buf, offset)
        x = ArgMaxOptions()
        x.Init(buf, n + offset)
        return x

    @classmethod
    def GetRootAsArgMaxOptions(cls, buf, offset=0):
        """This method is deprecated. Please switch to GetRootAs."""
        return cls.GetRootAs(buf, offset)
    @classmethod
    def ArgMaxOptionsBufferHasIdentifier(cls, buf, offset, size_prefixed=False):
        return flatbuffers.util.BufferHasIdentifier(buf, offset, b"\x54\x46\x4C\x33", size_prefixed=size_prefixed)

    # ArgMaxOptions
    def Init(self, buf, pos):
        self._tab = flatbuffers.table.Table(buf, pos)

    # ArgMaxOptions
    def OutputType(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(4))
        if o != 0:
            return self._tab.Get(flatbuffers.number_types.Int8Flags, o + self._tab.Pos)
        return 0

def ArgMaxOptionsStart(builder):
    builder.StartObject(1)

def Start(builder):
    ArgMaxOptionsStart(builder)

def ArgMaxOptionsAddOutputType(builder, outputType):
    builder.PrependInt8Slot(0, outputType, 0)

def AddOutputType(builder, outputType):
    ArgMaxOptionsAddOutputType(builder, outputType)

def ArgMaxOptionsEnd(builder):
    return builder.EndObject()

def End(builder):
    return ArgMaxOptionsEnd(builder)


class ArgMaxOptionsT(object):

    # ArgMaxOptionsT
    def __init__(self):
        self.outputType = 0  # type: int

    @classmethod
    def InitFromBuf(cls, buf, pos):
        argMaxOptions = ArgMaxOptions()
        argMaxOptions.Init(buf, pos)
        return cls.InitFromObj(argMaxOptions)

    @classmethod
    def InitFromPackedBuf(cls, buf, pos=0):
        n = flatbuffers.encode.Get(flatbuffers.packer.uoffset, buf, pos)
        return cls.InitFromBuf(buf, pos+n)

    @classmethod
    def InitFromObj(cls, argMaxOptions):
        x = ArgMaxOptionsT()
        x._UnPack(argMaxOptions)
        return x

    # ArgMaxOptionsT
    def _UnPack(self, argMaxOptions):
        if argMaxOptions is None:
            return
        self.outputType = argMaxOptions.OutputType()

    # ArgMaxOptionsT
    def Pack(self, builder):
        ArgMaxOptionsStart(builder)
        ArgMaxOptionsAddOutputType(builder, self.outputType)
        argMaxOptions = ArgMaxOptionsEnd(builder)
        return argMaxOptions
