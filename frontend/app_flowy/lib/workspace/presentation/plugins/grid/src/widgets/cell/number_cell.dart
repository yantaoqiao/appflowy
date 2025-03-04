import 'dart:async';

import 'package:app_flowy/startup/startup.dart';
import 'package:app_flowy/workspace/application/grid/prelude.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';

import 'cell_builder.dart';

class NumberCell extends GridCellWidget {
  final GridCellContextBuilder cellContextBuilder;

  NumberCell({
    required this.cellContextBuilder,
    Key? key,
  }) : super(key: key);

  @override
  State<NumberCell> createState() => _NumberCellState();
}

class _NumberCellState extends State<NumberCell> {
  late NumberCellBloc _cellBloc;
  late TextEditingController _controller;
  late CellSingleFocusNode _focusNode;
  Timer? _delayOperation;

  @override
  void initState() {
    final cellContext = widget.cellContextBuilder.build();
    _cellBloc = getIt<NumberCellBloc>(param1: cellContext)..add(const NumberCellEvent.initial());
    _controller = TextEditingController(text: _cellBloc.state.content);
    _focusNode = CellSingleFocusNode();
    _listenFocusNode();
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    _listenCellRequestFocus(context);
    return BlocProvider.value(
      value: _cellBloc,
      child: BlocConsumer<NumberCellBloc, NumberCellState>(
        listener: (context, state) {
          if (_controller.text != state.content) {
            _controller.text = state.content;
          }
        },
        builder: (context, state) {
          return TextField(
            controller: _controller,
            focusNode: _focusNode,
            onEditingComplete: () => _focusNode.unfocus(),
            maxLines: null,
            style: const TextStyle(fontSize: 14, fontWeight: FontWeight.w500),
            decoration: const InputDecoration(
              contentPadding: EdgeInsets.zero,
              border: InputBorder.none,
              isDense: true,
            ),
          );
        },
      ),
    );
  }

  @override
  Future<void> dispose() async {
    widget.requestFocus.removeAllListener();
    _delayOperation?.cancel();
    _cellBloc.close();
    _focusNode.removeSingleListener();
    _focusNode.dispose();
    super.dispose();
  }

  @override
  void didUpdateWidget(covariant NumberCell oldWidget) {
    if (oldWidget != widget) {
      _listenFocusNode();
    }
    super.didUpdateWidget(oldWidget);
  }

  Future<void> focusChanged() async {
    if (mounted) {
      _delayOperation?.cancel();
      _delayOperation = Timer(const Duration(milliseconds: 300), () {
        if (_cellBloc.isClosed == false && _controller.text != _cellBloc.state.content) {
          final number = num.tryParse(_controller.text);
          if (number != null) {
            _cellBloc.add(NumberCellEvent.updateCell(_controller.text));
          } else {
            _controller.text = "";
          }
        }
      });
    }
  }

  void _listenFocusNode() {
    widget.onFocus.value = _focusNode.hasFocus;
    _focusNode.setSingleListener(() {
      widget.onFocus.value = _focusNode.hasFocus;
      focusChanged();
    });
  }

  void _listenCellRequestFocus(BuildContext context) {
    widget.requestFocus.addListener(() {
      if (_focusNode.hasFocus == false && _focusNode.canRequestFocus) {
        FocusScope.of(context).requestFocus(_focusNode);
      }
    });
  }
}
