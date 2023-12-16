import flatbuffers
import tflite.Model as md 
import tflite.Tensor as ten

import tflite

if __name__ == '__main__':
    # read file as binary 
    with open('lite-model_movenet_singlepose_lightning_tflite_int8_4.tflite', 'rb') as f:
        ba = bytearray(f.read())
        original_model = tflite.Model.Model.GetRootAsModel(ba, 0)
        lower_model = tflite.Model.Model.GetRootAsModel(ba, 0)
        upper_model = tflite.Model.Model.GetRootAsModel(ba, 0)

        original_model = md.ModelT.InitFromObj(original_model)
        lower_model = md.ModelT.InitFromObj(lower_model)
        upper_model = md.ModelT.InitFromObj(upper_model)
    

        lower_model.subgraphs[0].operators = original_model.subgraphs[0].operators[:6]
        upper_model.subgraphs[0].operators = original_model.subgraphs[0].operators[6:]

        buf = tflite.Buffer.BufferT()
        upper_model.buffers.append(buf)
        lower_model.buffers.append(buf)

        deq_tensor = ten.TensorT()
        deq_tensor.shape = [1,96,96,32]
        deq_tensor.type = 0
        deq_tensor.buffer = 335
        deq_tensor.name = b'StatefulPartitionedCall:0'

        quant_tensor = ten.TensorT()
        quant_tensor.shape = [1,96,96,32]
        quant_tensor.type = 0
        quant_tensor.buffer = 335
        quant_tensor.name = b'StatefulPartitionedCall:0'

        lower_model.subgraphs[0].tensors.append(deq_tensor)
        # upper_model.subgraphs[0].tensors = [quant_tensor] + upper_model.subgraphs[0].tensors
        upper_model.subgraphs[0].tensors.append(quant_tensor)

        # print(upper_model.subgraphs[0].outputs)
        # print(lower_model.subgraphs[0].outputs)
        upper_model.subgraphs[0].inputs = [333]
        lower_model.subgraphs[0].outputs = [333]

        deq_op = tflite.Operator.OperatorT()
        deq_op.opcode_index = 13
        deq_op.inputs = [179]
        deq_op.outputs = [333]

        quant_op = tflite.Operator.OperatorT()
        quant_op.opcode_index = 1
        quant_op.inputs = [333]
        quant_op.outputs = [179]

        # upper_model.subgraphs[0].operators = [quant_op] + upper_model.subgraphs[0].operators
        upper_model.subgraphs[0].operators.append(quant_op)
        lower_model.subgraphs[0].operators.append(deq_op)
        
        builder = flatbuffers.Builder(1024)

        packed = lower_model.Pack(builder)
        builder.Finish(packed, b"TFL3")
        with open('lower_model.tflite', 'wb') as f:
            f.write(builder.Output())

        packed = upper_model.Pack(builder)
        builder.Finish(packed, b"TFL3")
        with open('upper_model.tflite', 'wb') as f:
            f.write(builder.Output())
