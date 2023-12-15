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
        
        original_subgraphs = original_model.subgraphs
        upper_subgraphs = upper_model.subgraphs
        lower_subgraphs = lower_model.subgraphs

        original_subgraph = original_subgraphs[0]
        lower_subgraph = lower_subgraphs[0]
        upper_subgraph = upper_subgraphs[0]

        upper_operators = upper_subgraph.operators[0:6]
        lower_operators = lower_subgraph.operators[6:]

        buf = tflite.Buffer.BufferT()
        upper_buffers = upper_model.buffers
        upper_buffers.append(buf)

        lower_buffers = lower_model.buffers
        lower_buffers.append(buf)

        deq_tensor = ten.TensorT()
        deq_tensor.shape = [1,96,96,3]
        deq_tensor.type = 0
        deq_tensor.buffer = 335
        deq_tensor.name = b'StatefulPartitionedCall:0'
        upper_tensors = upper_subgraph.tensors
        upper_tensors.append(deq_tensor) 

        quant_tensor = ten.TensorT()
        quant_tensor.shape = [1,96,96,3]
        quant_tensor.type = 0
        quant_tensor.buffer = 335
        quant_tensor.name = b'StatefulPartitionedCall:0'
        lower_tensors = lower_subgraph.tensors
        lower_tensors.append(quant_tensor)

        upper_subgraph.outputs = [333]
        lower_subgraph.outputs = [333]

        deq_op = tflite.Operator.OperatorT()
        deq_op.opcode_index = 13
        deq_op.inputs = [179]
        deq_op.outputs = [333]

        quant_op = tflite.Operator.OperatorT()
        quant_op.opcode_index = 1
        quant_op.inputs = [333]
        quant_op.outputs = [179]

        upper_subgraph.operators.append(deq_op)
        lower_subgraph.operators.append(quant_op)
        
        builder = flatbuffers.Builder(1024)
        packed = lower_model.Pack(builder)
        builder.Finish(packed)
        with open('lower_model.tflite', 'wb') as f:
            f.write(builder.Output())
            
        packed = upper_model.Pack(builder)
        builder.Finish(packed)
        with open('upper_model.tflite', 'wb') as f:
            f.write(builder.Output())
